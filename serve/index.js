(() => {
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

    async function refresh()
    {
        const response = await fetch("/api/rows?order-by=description&desc=on");
        const rows = await response.json();
        for (const row of rows) {
            showMethod(row);
        }
    }

    function init() {
        console.log("Henlo!");
        refresh();
    }

    window.addEventListener("load", init);
})();