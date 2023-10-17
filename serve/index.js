(() => {
    const UP = "▲";
    const DOWN = "▼";
    const COLUMN_NAMES = ["id", "name", "description", "image"];
    
    function hideMethods()
    {
        const tbodyMethods = document.getElementById("tbody-methods");
        tbodyMethods.replaceChildren();
    }

    function showMethod(method)
    {
        const tbodyMethods = document.getElementById("tbody-methods");
        const templateMethodRow = document.getElementById("template-method-row");
        const rowMethod = document.importNode(templateMethodRow.content.querySelector("tr"), true);

        rowMethod.querySelector(".column-id").textContent = method.id;
        rowMethod.querySelector(".column-name").textContent = method.name;
        rowMethod.querySelector(".column-description").textContent = method.description;
        rowMethod.querySelector(".column-image").textContent = method.image;
        
        tbodyMethods.appendChild(rowMethod);
    }
    // : ; and ▼: &#9660;
    function getOrderParams() {
        let activeColumn = "id";
        let activeDirection = UP;
        
        for (const columnName of COLUMN_NAMES) {
            const t = document.getElementById(`span-${columnName}-header`).textContent;
            if (t != '')
            {
                activeColumn = columnName;
                activeDirection = t;
            }
        }
        
        let order = { order_by: activeColumn, };
        if (activeDirection === DOWN) order.desc = 'on';
        
        return new URLSearchParams(order);
    }

    async function refresh()
    {
        const orderParams = getOrderParams();

        const response = await fetch("/api/rows?" + orderParams);
        const rows = await response.json();

        hideMethods();
        for (const row of rows) {
            showMethod(row);
        }
    }

    // TODO why does this sometimes fail when I click the ID button after refreshing the page? Does not need to be immediately after refresh.
    // In failure, instead of the ID button's span being updated, it just empties as though none of the spans are equal.
    // Hasn't shown up since I moved to testing columnSpan.columnname instead of columnSpan directly; might be fixed.
    function onButtonColumn(event) {
        const myName = event.target.columnSpan.columnName;//console.log(event.target.columnSpan.columnName);
        for (const columnName of COLUMN_NAMES) {
            const span = document.getElementById(`span-${columnName}-header`);
            if (span.columnName === myName) {
                span.textContent = span.textContent === UP ? DOWN : UP;
            } else {
                span.textContent = "";
            }
        }
        refresh();
    }

    function init() {
        for (const columnName of COLUMN_NAMES) {
            const span = document.getElementById(`span-${columnName}-header`);
            const button = document.getElementById(`button-${columnName}-header`);
            button.columnSpan = span;
            span.columnName = columnName;

            button.addEventListener('click', onButtonColumn);
        }
        refresh();
    }

    window.addEventListener("load", init);
})();