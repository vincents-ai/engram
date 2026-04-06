// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="development/DEVELOPMENT.html"><strong aria-hidden="true">1.</strong> Development Guide</a></li><li class="chapter-item expanded "><a href="getting-started.html"><strong aria-hidden="true">2.</strong> Quick Start</a></li><li class="chapter-item expanded "><a href="user-guide.html"><strong aria-hidden="true">3.</strong> User Guide</a></li><li class="chapter-item expanded "><a href="architecture/overview.html"><strong aria-hidden="true">4.</strong> Architecture Overview</a></li><li class="chapter-item expanded "><a href="concepts/tasks.html"><strong aria-hidden="true">5.</strong> Tasks</a></li><li class="chapter-item expanded "><a href="concepts/context.html"><strong aria-hidden="true">6.</strong> Context</a></li><li class="chapter-item expanded "><a href="concepts/reasoning.html"><strong aria-hidden="true">7.</strong> Reasoning</a></li><li class="chapter-item expanded "><a href="concepts/knowledge.html"><strong aria-hidden="true">8.</strong> Knowledge</a></li><li class="chapter-item expanded "><a href="concepts/relationships.html"><strong aria-hidden="true">9.</strong> Relationships</a></li><li class="chapter-item expanded "><a href="features/theory-building.html"><strong aria-hidden="true">10.</strong> Theory Building</a></li><li class="chapter-item expanded "><a href="features/state-reflection.html"><strong aria-hidden="true">11.</strong> State Reflection</a></li><li class="chapter-item expanded "><a href="features/example-workflows.html"><strong aria-hidden="true">12.</strong> Example Workflows</a></li><li class="chapter-item expanded "><a href="features/workflows.html"><strong aria-hidden="true">13.</strong> Workflows</a></li><li class="chapter-item expanded "><a href="features/sessions.html"><strong aria-hidden="true">14.</strong> Sessions</a></li><li class="chapter-item expanded "><a href="skills/using-engram.html"><strong aria-hidden="true">15.</strong> Using Engram (for Agents)</a></li><li class="chapter-item expanded "><a href="skills/prompt_guide.html"><strong aria-hidden="true">16.</strong> Prompt Guide</a></li><li class="chapter-item expanded "><a href="reference/cli.html"><strong aria-hidden="true">17.</strong> CLI Reference</a></li><li class="chapter-item expanded "><a href="reference/entities.html"><strong aria-hidden="true">18.</strong> Entity Schema</a></li><li class="chapter-item expanded "><a href="reference/api.html"><strong aria-hidden="true">19.</strong> API Reference</a></li><li class="chapter-item expanded "><a href="development/setup.html"><strong aria-hidden="true">20.</strong> Setup &amp; Configuration</a></li><li class="chapter-item expanded "><a href="validation.html"><strong aria-hidden="true">21.</strong> Validation</a></li><li class="chapter-item expanded "><a href="development/contributing.html"><strong aria-hidden="true">22.</strong> Contributing</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
