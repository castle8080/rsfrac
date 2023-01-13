// Just enough javascript to handle clicking in the image to recenter and zoom in.
class MandelbrotImageUX {
    constructor() {
        this.image = document.getElementById("mandelbrot_image");
        this.start_color = document.getElementById("start_color");
        this.end_color = document.getElementById("end_color");

        // Check the page URI for parameters to give to the image.
        if (window.location.search.length > 0) {
            this.image.src = "/images/mandelbrot" + window.location.search;
        }

        const img_params = this.get_image_params();
        console.log(img_params);
        if (img_params.start_color) {
            this.start_color.value = img_params.start_color;
        }
        if (img_params.end_color) {
            this.end_color.value = img_params.end_color;
        }
        
        this.image.addEventListener('click', (event) => this.on_image_click(event));

        this.start_color.addEventListener('change', (event) => this.on_control_change());
        this.end_color.addEventListener('change', (event) => this.on_control_change());

        document.getElementById("swap_colors").addEventListener('click', (event) => {
            event.preventDefault();
            this.swap_colors();
            return false;
        });
    }

    swap_colors() {
        let start_color = this.start_color.value;
        let end_color = this.end_color.value;
        this.start_color.value = end_color;
        this.end_color.value = start_color;
        this.on_control_change();
    }

    on_image_click(event) {
        console.log(this.image);
        const x = event.pageX - this.image.offsetLeft;
        const y = event.pageY - this.image.offsetTop;

        const img_params = this.get_image_params();
        const new_img_params = { ... img_params };

        const x_delta_ratio = (x - (this.image.width / 2)) / this.image.width;
        const y_delta_ratio = (y - (this.image.height / 2)) / this.image.height;

        new_img_params.x = img_params.x + x_delta_ratio * img_params.width;
        new_img_params.y = img_params.y - y_delta_ratio * img_params.height;

        new_img_params.width = img_params.width / 2;
        new_img_params.height = img_params.height / 2;

        new_img_params.start_color = this.start_color.value;
        new_img_params.end_color = this.end_color.value;

        this.set_new_img_params(this.get_uri_params(new_img_params));
    }

    on_control_change() {
        const img_params = this.get_image_params();
        const new_img_params = { ... img_params };

        new_img_params.start_color = this.start_color.value;
        new_img_params.end_color = this.end_color.value;

        this.set_new_img_params(this.get_uri_params(new_img_params));
    }

    set_new_img_params(uri_params) {
        // Update the image uri and update the page uri so you can back here.
        this.image.src = "/images/mandelbrot?" + uri_params;
        window.history.replaceState({}, "", "/mandelbrot.html?" + uri_params);
    }

    get_uri_params(img_params) {
        const params = [];
        for (const k in img_params) {
            params.push(encodeURIComponent(k) + "=" + encodeURIComponent(img_params[k].toString()));
        }
        return params.join("&");
    }

    get_image_params() {
        let params = {};
        let kv_pairs = this.image.src.substring(this.image.src.indexOf('?') + 1).split("&");
        kv_pairs.forEach(kv => {
            let pair = kv.split('=');
            let key = decodeURIComponent(pair[0]);
            let val = decodeURIComponent(pair[1]);
            if (!key.includes("color")) {
                val = parseFloat(val);
            }
            params[key] = val;
        });
        return params;
    }

}

addEventListener('DOMContentLoaded', (event) => {
    new MandelbrotImageUX()
});