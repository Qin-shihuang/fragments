function renderPostContent(text) {
    return text.replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/\n/g, '<br>')
        .replace(/ /g, '&nbsp;')
        .replace(/\[([^\]]*)\]\(([^)]+)\)/g, (match, text, url) => {
            if (text === "") {
                text = "link";
            }            
            const escapedUrl = url.replace(/"/g, '&quot;');
            return `<a class="post-inline-link" href="${escapedUrl}" target="_blank">[${text}]</a>`;
        })
        .replace(/#(\d+)/g, (match, id) => {
            return `<a class="post-inline-link" href="/post/${id}">#${id}</a>`;
        })
        .replace(/'/g, '&#039;');
}