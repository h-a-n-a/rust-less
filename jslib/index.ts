import less from "less";

function get_argv(key: string) {
    let list = process.argv;
    let index = list.findIndex((p) => {
        return p == "--" + key
    })
    if (index > -1) {
        return process.argv[index + 1]
    }
}

async function main() {
    let content = get_argv("content");
    let option_value = get_argv("option");
    let options = undefined;
    if (option_value) {
        options = JSON.parse(option_value);
    }
    if (content) {
        content = JSON.parse(content)?.content;
    }
    if (content) {
        less.render(content, {
            paths: [
                ...(options?.paths || ['node_modules']),
                ...(options?.root ? [options.root] : []),
            ],
        }).then(res => {
            console.log(res.css)
            if (process.send) {
                process.send?.(res.css);
            }
            process.exit(0);
        }).catch(ex => {
            console.log(ex);
            process.exit(1);
        })

    }
}

main();
