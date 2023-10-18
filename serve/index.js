(() => {
    const UP = "▲"; // &#9650
    const DOWN = "▼"; // &#9660
    const COLUMN_NAMES = ["id", "name", "description", "image"];
    
    function hideMethods()
    {
        const tbodyMethods = document.getElementById("tbody-methods");
        tbodyMethods.replaceChildren();
    }

    function onFieldKeyup() {
        refresh();
    }

    function showMethod(method)
    {
        const tbodyMethods = document.getElementById("tbody-methods");
        const templateMethodRow = document.getElementById("template-method-row");
        const rowMethod = document.importNode(templateMethodRow.content.querySelector("tr"), true);

        rowMethod.querySelector(".column-id").textContent = method.id;
        rowMethod.querySelector(".column-name").textContent = method.name;
        rowMethod.querySelector(".column-description").textContent = method.description;

        const image = document.createElement("img");
        image.src = method.image;
        rowMethod.querySelector(".column-image").appendChild(image);
        
        const buttonDelete = rowMethod.querySelector(".button-delete");
        buttonDelete.methodId = method.id;
        buttonDelete.addEventListener("click", onButtonDelete);
        
        const buttonEdit = rowMethod.querySelector(".button-edit");
        buttonEdit.methodId = method.id;
        buttonEdit.addEventListener("click", onButtonEdit);
        
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
        
        return order;
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
        const url = "/api/rows?" + params;
        console.log("URL: " + url);
        const response = await fetch(url);
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

    function onButtonEditRow(event) {
        console.log("Editing");
        event.preventDefault();
        const form = document.getElementById("form-edit-row");
        const formData = new URLSearchParams(new FormData(form));
        const url = "/api/row?" + formData;
        console.log("PUT " + url);
        fetch(url, {
            method: "PUT",
        });
        refresh();
        document.getElementById("non-edit-stuff").style.display = "block";
        document.getElementById("form-edit-row").style.display = "none";
    }

    function onButtonEdit(event) {
        document.getElementById("field-edit-id").value = event.target.parentElement.parentElement.querySelector(".column-id").textContent;
        document.getElementById("field-edit-name").value = event.target.parentElement.parentElement.querySelector(".column-name").textContent;
        document.getElementById("field-edit-description").value = event.target.parentElement.parentElement.querySelector(".column-description").textContent;
        document.getElementById("field-edit-image").value = event.target.parentElement.parentElement.querySelector(".column-image").querySelector("img").src;
        document.getElementById("form-edit-row").style.display = "block";
        document.getElementById("non-edit-stuff").style.display = "none";
    }

    function onButtonDelete(event) {
        const id = event.target.methodId;
        const formData = new URLSearchParams({"field-delete-id": id});
        const url = "/api/row?" + formData;
        console.log("DELETE THIS " + url);
        fetch(url, {
            method: "DELETE",
        });
        refresh();

    }

    function onButtonCreateRow(event) {
        event.preventDefault();
        const form = document.getElementById("form-create-row");
        const formData = new URLSearchParams(new FormData(form));
        // for (const entry of formData.entries()) {
        //     console.log(entry);
        // }
        const url = "/api/row?" + formData;
        console.log("Posting " + url);
        fetch(url, {
            method: "POST",
            // body: formData,
        });
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

        const buttonCreateRow = document.getElementById("button-create-row");
        buttonCreateRow.addEventListener("click", onButtonCreateRow);
        const buttonEditRow = document.getElementById("button-edit-row");
        buttonEditRow.addEventListener("click", onButtonEditRow);
        const fieldId = document.getElementById("field-id");
        fieldId.addEventListener("keyup", onFieldKeyup);
        const fieldKeyword = document.getElementById("field-keyword");
        fieldKeyword.addEventListener("keyup", onFieldKeyup);
        
        refresh();
    }

    window.addEventListener("load", init);
})();