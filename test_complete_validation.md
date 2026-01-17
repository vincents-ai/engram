# Test File for Complete Validation

This file demonstrates the complete validation workflow with:
- Task ID extraction from commit message
- Task existence verification 
- Relationship validation (reasoning + context)
- Successful pre-commit hook validation

The validation system successfully:
1. Extracts UUID task IDs from commit messages
2. Validates task exists in storage 
3. Checks for required relationships
4. Passes validation when all criteria are met