let base_path = "${BASE_PATH}";

window.conduct = {
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
    }
}
