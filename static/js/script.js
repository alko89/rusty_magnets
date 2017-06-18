jQuery(function($){
    magnetSearch();

    $('#search').keyup(function(){
        magnetSearch();
    });

    function magnetSearch(){
        var txt = $('#search').val();
        if (txt == "")
            txt = " ";

        $.ajax({
            type: "GET",
            url: "/magnet/search/" + txt + "/0/100"
        }).done(function(result) {
            $("#table tbody").empty();
            
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
});
