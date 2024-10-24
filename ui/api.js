let base_path = "${BASE_PATH}";

window.conduct = {
    get: function (path) {
        return fetch(`${base_path}/${path}`);
    }
};
