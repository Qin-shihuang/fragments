function fetchAndDisplayAllPosts() {
    fetch('/api/posts')
        .then(response => response.json())
        .then(groupedPosts => {
            const postsDiv = document.getElementById('posts');
            postsDiv.innerHTML = '';
            groupedPosts.forEach(group => {
                const dateHeader = document.createElement('h2');
                dateHeader.textContent = group.date;
                postsDiv.appendChild(dateHeader);
                
                const ul = document.createElement('ul');
                group.posts.forEach(post => ul.appendChild(createPostElement(post)));
                postsDiv.appendChild(ul);
            });
        })
        .catch(error => {
            console.error('Error fetching posts:', error);
            const postsDiv = document.getElementById('posts');
            postsDiv.innerHTML = '<p>Error loading posts. Please try again later.</p>';
        });
}

function createPostElement(post) {
    const li = document.createElement('li');
    const div = document.createElement('div');
    div.id = `post-${post.id}`;
    div.className = 'post-item';
    div.innerHTML = `<strong>${post.sentence}</strong>`;
    div.style.cursor = 'pointer';
    div.addEventListener('click', () => {
        window.location.href = `/post/${post.id}`;
    });
    li.appendChild(div);
    return li;
}