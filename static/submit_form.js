$("#form").on("submit", function(event) {
    event.preventDefault();

    var formData = {
        paperless_url: $("#paperless_url").val(),
        paperless_token: $("#paperless_token").val(),
        asn_from: $("#asn_from").val(),
        asn_to: $("#asn_to").val(),
        show_fields: $("#show_fields").val(),
        group_by: $("#group_by").val(),
        sort_by: $("#sort_by").val(),
        sort_desc: $("#sort_desc").is(":checked"),
    };
    
    var json = JSON.stringify(formData);

    $.ajax({
        type: "POST",
        url: "/show_index",
        data: json,
        success: function(response) {
            var newWindow = window.open("", "new window");
            newWindow.document.write(response);
        },
        dataType: "html",
        contentType : "application/json"
    })
});
