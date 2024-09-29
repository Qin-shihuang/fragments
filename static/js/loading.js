const LoadingManager = (function () {
    let hiddenElements = [];

    return {
        showLoading: function (elems) {
            document.getElementById('loading').style.display = 'block';
            hiddenElements = elems.map(elem => {
                const element = document.getElementById(elem);
                if (!element) {
                    return;
                }
                const currentDisplay = element.style.display;
                element.style.display = 'none';
                return { id: elem, display: currentDisplay };
            });
        },

        hideLoading: function () {
            document.getElementById('loading').style.display = 'none';
            hiddenElements.forEach((kv) => {
                if (!kv) {
                    return;
                }
                const { id, display } = kv;
                const element = document.getElementById(id);
                if (element) {
                    element.style.display = display;
                }
            });
            hiddenElements = [];
        }
    };
})();