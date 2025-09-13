let base_path = "${BASE_PATH}";

window.conduct = {

    url_base_path: function () {
        return base_path
    },

    get: function (path) {
        return fetch(`${base_path}/${path}`);
    },

    post: function (path, body) {
        return fetch(`${base_path}/${path}`, {
            method: "POST",
            body: body
        });
    }
};

window.os = {
    execute: function (command) {
        return fetch(`${base_path}/os/execute`, {
            method: "POST",
            body: command
        });
    },

    file: function (path) {
        return fetch(`${base_path}/os/file?path=${encodeURIComponent(path)}`, {
            method: "GET",
        });
    },
}
