jQuery(function($){
    var page = 0;
    var pages;
    var items_per_page = 10;
    var txt = "";

    magnetSearch();

    $('#search').keyup(function(){
        magnetSearch();
    });
//     $(".page").on("click", function() {
//         page = this.value;
//         magnetSearch();
//     })
    $("#items").on("change", function() {
        items_per_page = this.value;
        magnetSearch();
    })

    function magnetSearch(){
        txt = $('#search').val();
        if (txt == "")
            txt = " ";

        $.ajax({
            type: "GET",
            url: "/magnet/search/" + txt + "/" + page + "/" + items_per_page
        }).done(function(result) {
            $("#table tbody").empty();

            pagination(result.magnets_filtered);
            
            $.each(result.magnets, function(i, item) {
                var $tr = $('<tr>').append(
                    $('<td>').text(item.name),
                    $('<td>').text(item.size),
                    $('<td>').text(item.seeders),
                    $('<td>').text(item.leechers),
                    
                    $('<td>').html('<a href="' + item.magnet + '"><img src="/images/icon-magnet.png" width="30" height="30" /></a>'),
                    $('<td>').html('<a href="' + item.url  + '">' + item.website_source + '</a>')
                ).appendTo("#table tbody");
            });
        });
    }
    
    function pagination(filtered){
            pages = filtered / items_per_page;
            $("#pages").empty();
            
            if (pages > 10){
                start = 0;
                end = pages;
                //TODO
            }
            
            for (var i = 0; i < pages; i++){
                if (i == page)
                    var button = $('<button/>', {
                        type: 'button',
                        'class': 'pure-button pure-button-disabled',
                        text: i + 1,
                        value: i,
                        click: function() {
                            page = this.value;
                            magnetSearch();
                        }
                    });
                else
                    var button = $('<button/>', {
                        type: 'button',
                        'class': 'pure-button',
                        text: i + 1,
                        value: i,
                        click: function() {
                            page = this.value;
                            magnetSearch();
                        }
                    });
                $("#pages").append(button);
            }
    }
});
