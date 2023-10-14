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

    function init() {
        console.log("Henlo!");
        showMethod({id: 12, name: 413, description: "Payload too large", image: "Cathy shoving things in closet"});
    }

    window.addEventListener("load", init);
})();