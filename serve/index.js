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
        const response = await fetch("/api/rows");
        const result = await response.text();
        console.log(result);
    }

    function init() {
        console.log("Henlo!");
        showMethod({id: 12, name: 413, description: "Payload too large", image: "Cathy shoving things in closet"});
        refresh();
    }

    window.addEventListener("load", init);
})();