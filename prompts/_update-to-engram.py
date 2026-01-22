#!/usr/bin/env python3
"""
Bulk update Goose agent prompts to engram-adapted format.

This script processes all YAML files in the agents directory and updates them
to include engram integration patterns.
"""

import os
import re
import json
import yaml
from pathlib import Path


def add_engram_instructions(original_instructions: str) -> str:
    """Add engram integration instructions to agent."""
    engram_prefix = """ENGRAM INTEGRATION - YOU MUST:

1. BEFORE WORK:
   ```bash
   engram task show {{task_id}}
   engram relationship connected --entity-id {{task_id}} --references
   ```

2. DURING WORK:
   ```bash
   engram reasoning create --title "[Work] Progress" --task-id {{task_id}} --content "[What you did]"
   ```

3. AFTER WORK:
   ```bash
   engram context create --title "[Result]" --content "[Your output]"
   engram relationship create --source-id {{task_id}} --target-id [RESULT_ID] --produces
   engram task update {{task_id}} --status done --outcome "[Summary]"
   ```

"""
    
    # Check if already has engram integration
    if "engram" in original_instructions.lower():
        return original_instructions
    
    return engram_prefix + original_instructions


def update_agent_parameters(parameters: dict) -> dict:
    """Update parameters to include task_id."""
    # Add task_id parameter if not exists
    if "properties" in parameters:
        if "task_id" not in parameters["properties"]:
            parameters["properties"]["task_id"] = {
                "type": "string",
                "description": "The engram task ID for this work"
            }
        if "required" in parameters and "task_id" not in parameters["required"]:
            parameters["required"].append("task_id")
    return parameters


def update_agent_prompt(prompt: str) -> str:
    """Update prompt to include engram workflow steps."""
    engram_prompt_addition = """

ENGRAM WORKFLOW:
1. Get task: `engram task show {{task_id}}`
2. Get context: `engram relationship connected --entity-id {{task_id}} --references`
3. Store progress: `engram reasoning create --title "[Progress]" --task-id {{task_id}} --content "[What you did]"`
4. Store result: `engram context create --title "[Result]" --content "[Output]"`
5. Complete: `engram task update {{task_id}} --status done --outcome "[Summary]"`

Return JSON with task_id, status, and result_summary.
"""
    
    if "engram" in prompt.lower():
        return prompt
    
    return prompt + engram_prompt_addition


def update_response_schema(response: dict) -> dict:
    """Update response to include engram fields."""
    if "schema" in response and "properties" in response["schema"]:
        schema = response["schema"]["properties"]
        
        # Ensure required fields exist
        if "task_id" not in schema:
            schema["task_id"] = {
                "type": "string",
                "description": "The task ID (echoed for confirmation)"
            }
        if "status" not in schema:
            schema["status"] = {
                "type": "string",
                "description": "Status of work completion"
            }
        
        # Update required fields
        if "required" in response["schema"]:
            if "task_id" not in response["schema"]["required"]:
                response["schema"]["required"].append("task_id")
            if "status" not in response["schema"]["required"]:
                response["schema"]["required"].append("status")
    
    return response


def process_agent_file(filepath: Path) -> bool:
    """Process a single agent YAML file."""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        # Parse YAML
        data = yaml.safe_load(content)
        if not data:
            return False
        
        # Track changes
        changed = False
        
        # Update instructions
        if "instructions" in data:
            original = data["instructions"]
            data["instructions"] = add_engram_instructions(original)
            if data["instructions"] != original:
                changed = True
        
        # Update parameters
        if "parameters" in data:
            original = str(data["parameters"])
            data["parameters"] = update_agent_parameters(data["parameters"])
            if str(data["parameters"]) != original:
                changed = True
        
        # Update prompt
        if "prompt" in data:
            original = data["prompt"]
            data["prompt"] = update_agent_prompt(original)
            if data["prompt"] != original:
                changed = True
        
        # Update response
        if "response" in data:
            original = str(data["response"])
            data["response"] = update_response_schema(data["response"])
            if str(data["response"]) != original:
                changed = True
        
        # Update title
        if "title" in data and "(Engram-Adapted)" not in data["title"]:
            data["title"] = data["title"] + " (Engram-Adapted)"
            changed = True
        
        # Write back if changed
        if changed:
            with open(filepath, 'w') as f:
                yaml.dump(data, f, default_flow_style=False, sort_keys=False)
            return True
        
        return False
    
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False


def process_pipeline_file(filepath: Path) -> bool:
    """Process a single pipeline YAML file."""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        data = yaml.safe_load(content)
        if not data:
            return False
        
        changed = False
        
        # Update title
        if "title" in data and "(Engram-Adapted)" not in data["title"]:
            data["title"] = data["title"] + " (Engram-Adapted)"
            changed = True
        
        # Update instructions
        if "instructions" in data:
            if "engram" not in data["instructions"].lower():
                engram_instr = """
ENGRAM INTEGRATION:
- Create engram workflow for orchestration
- Use engram tasks for each stage
- Track progress via engram workflow status
- Store all outputs in engram entities

"""
                data["instructions"] = engram_instr + data["instructions"]
                changed = True
        
        # Update parameters
        if "parameters" in data:
            if "properties" in data["parameters"]:
                if "parent_task_id" not in data["parameters"]["properties"]:
                    data["parameters"]["properties"]["parent_task_id"] = {
                        "type": "string",
                        "description": "The engram parent task ID"
                    }
                    if "required" in data["parameters"]:
                        data["parameters"]["required"].append("parent_task_id")
                    changed = True
        
        # Update sub_recipes paths
        if "sub_recipes" in data:
            for recipe in data["sub_recipes"]:
                if "path" in recipe and "-engram-adapted" not in recipe["path"]:
                    recipe["path"] = recipe["path"].replace(".yaml", "-engram-adapted.yaml")
                    changed = True
        
        if changed:
            with open(filepath, 'w') as f:
                yaml.dump(data, f, default_flow_style=False, sort_keys=False)
            return True
        
        return False
    
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False


def main():
    """Process all agent and pipeline files."""
    base_path = Path(__file__).parent
    
    # Process agents
    agents_dir = base_path / "agents"
    if agents_dir.exists():
        agent_count = 0
        agent_changed = 0
        for filepath in agents_dir.glob("*.yaml"):
            if filepath.name.startswith("_"):
                continue  # Skip templates
            agent_count += 1
            if process_agent_file(filepath):
                agent_changed += 1
                print(f"Updated: {filepath.name}")
        
        print(f"\nAgents: {agent_changed}/{agent_count} updated")
    
    # Process pipelines
    pipelines_dir = base_path / "ai" / "pipelines"
    if pipelines_dir.exists():
        pipeline_count = 0
        pipeline_changed = 0
        for filepath in pipelines_dir.glob("*.yaml"):
            if filepath.name.startswith("_"):
                continue  # Skip templates
            pipeline_count += 1
            if process_pipeline_file(filepath):
                pipeline_changed += 1
                print(f"Updated: {filepath.name}")
        
        print(f"\nPipelines: {pipeline_changed}/{pipeline_count} updated")


if __name__ == "__main__":
    main()
