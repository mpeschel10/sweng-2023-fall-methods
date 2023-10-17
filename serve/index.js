(() => {
    const UP = "▲"; // &#9650
    const DOWN = "▼"; // &#9660
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
    function getOrder() {
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

    function getWhere() {
        const where = {};
        const id = document.getElementById("field-id").value;
        if (id !== "") { where.where_id = id; }
        const keyword = document.getElementById("field-keyword").value;
        if (keyword !== "") { where.where_keyword = `%${keyword}%`; }
        return where;
    }

    async function refresh()
    {
        const params = new URLSearchParams({...getOrder(), ...getWhere()})
        const response = await fetch("/api/rows?" + params);
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