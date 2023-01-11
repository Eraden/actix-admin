document.onkeydown = function (e) {
    switch (e.which) {
        case 37: // left
            let left_el = document.getElementsByClassName('left-arrow-click').item(0);
            if (left_el) { left_el.click(); };
            break;

        //case 38: // up
        //    break;

        case 39: // right
            let right_el = document.getElementsByClassName('right-arrow-click').item(0);
            if (right_el) { right_el.click(); };
            break;

        //case 40: // down
        //    break;

        default: return; // exit this handler for other keys
    }
    e.preventDefault(); // prevent the default action (scroll / move caret)
};

function checkAll(bx) {
    var cbs = document.getElementsByTagName('input');
    for (var i = 0; i < cbs.length; i++) {
        if (cbs[i].type == 'checkbox') {
            cbs[i].checked = bx.checked;
        }
    }
}

function sort_by(column) {
    document.getElementById("sort_by").value = column;
    current_sort_order = document.getElementById("sort_order").value;
    if (current_sort_order == "{{ sort_order_asc }}") {
        document.getElementById("sort_order").value = "{{ sort_order_desc }}";
    } else {
        document.getElementById("sort_order").value = "{{ sort_order_asc }}";
    }
    document.getElementById('search_form').submit();
}

document.addEventListener('DOMContentLoaded', () => {
    // Get all "navbar-burger" elements
    const $navbarBurgers = Array.prototype.slice.call(document.querySelectorAll('.navbar-burger'), 0);

    // Add a click event on each of them
    $navbarBurgers.forEach(el => {
        el.addEventListener('click', () => {

            // Get the target from the "data-target" attribute
            const target = el.dataset.target;
            const $target = document.getElementById(target);

            // Toggle the "is-active" class on both the "navbar-burger" and the "navbar-menu"
            el.classList.toggle('is-active');
            $target.classList.toggle('is-active');

        });
    });
});

htmx.on("htmx:responseError", function () {
    document.getElementById("notifications").insertAdjacentHTML(
        "afterend",
        "<div class=\"notification mb-4 is-light is-danger\"><button class=\"delete\" onclick=\"this.parentElement.remove()\"></button>An Error occurred</div>");
})