function preparePaginatedPage() {
    const urlParams = new URLSearchParams(window.location.search);
    const page = urlParams.get('page') || 1;
    const perPage = urlParams.get('per_page') || 20;
    fetchAndDisplayPosts(page, perPage);
}

function fetchAndDisplayPosts(page, perPage) {
    fetch(`/api/posts?page=${page}&per_page=${perPage}`)
        .then(response => response.json())
        .then(groupedPosts => {
            if (!groupedPosts || groupedPosts.length === 0) {
                const postsDiv = document.getElementById('posts');
                postsDiv.innerHTML = '<p>No posts match the selected criteria.</p>';
                return;
            }
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
    li.className = 'post-container';

    const postLink = document.createElement('a');
    postLink._href = `/post/${post.id}`;
    postLink.className = 'post-item';
    postLink.innerHTML = `<span>${post.sentence}</span>`;

    postLink.addEventListener('click', (event) => {
        event.preventDefault();
        window.location.href = postLink._href;
    });

    li.appendChild(postLink);
    return li;
}

function handleKeyboardNavigation(event) {
    if (event.key === 'ArrowLeft' || event.key === 'ArrowRight') {
        event.preventDefault();

        let link;
        if (event.key === 'ArrowLeft') {
            link = document.getElementById('prev');
        } else if (event.key === 'ArrowRight') {
            link = document.getElementById('next');
        }

        if (link && link.href) {
            window.location.href = link.href;
        }
    }
}

function setupKeyboardNavigation() {
    document.addEventListener('keydown', handleKeyboardNavigation);
}