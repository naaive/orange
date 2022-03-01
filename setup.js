const fs = require('fs');
fs.readFile("node_modules/semantic-ui-css/semantic.css", 'utf8', function (err, data) {
    if (err) {
        return console.log(err);
    }
    const result = data.replace(/;;/g, ';');

    fs.writeFile("node_modules/semantic-ui-css/semantic_copy.css", result, 'utf8', function (err) {
        if (err) return console.log(err);
    });
});