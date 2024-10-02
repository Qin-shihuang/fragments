function preparePaginatedPostsPage() {
    const urlParams = new URLSearchParams(window.location.search);
    const page = urlParams.get('page') || 1;
    const perPage = urlParams.get('per_page') || 20;
    const source = `/api/posts?page=${page}&per_page=${perPage}`;
    fetchAndDisplayPosts(source);
}

function prepareAllPostsPage() {
    const source = '/api/posts?page=0&per_page=0'
    fetchAndDisplayPosts(source);
}

function prepareDatePostsPage() {
    const date = window.location.pathname.split('/').pop();
    if (!/^\d{4}-\d{2}-\d{2}$/.test(date) || isNaN(new Date(date))) {
        const postsDiv = document.getElementById('posts');
        postsDiv.innerHTML = '<center>Invalid date format.</center>';
        return;
    }
    const source = `/api/posts/${date}`;
    fetchAndDisplayPosts(source);
}

function fetchAndDisplayPosts(source) {
    fetch(source)
        .then(response => response.json())
        .then(groupedPosts => {
            if (!groupedPosts || groupedPosts.length === 0) {
                const postsDiv = document.getElementById('posts');
                postsDiv.innerHTML = '<center>No posts match the selected criteria.</center>';
                return;
            }
            const postsDiv = document.getElementById('posts');
            postsDiv.innerHTML = '';
            groupedPosts.forEach(group => {
                const dateHeader = document.createElement('h2');
                const hr = document.createElement('hr');
                hr.className = 'h2-hr';
                const target = `/date/${group.date}`;
                if (window.location.pathname !== target) {
                    const dateLink = document.createElement('a');
                    dateLink.href = target
                    dateLink.textContent = group.date;
                    dateLink.className = 'date-link';
                    dateHeader.appendChild(dateLink);
                } else {
                    dateHeader.textContent = group.date;
                }
                postsDiv.appendChild(dateHeader);
                postsDiv.appendChild(hr);

                const ul = document.createElement('ul');
                group.posts.forEach(post => ul.appendChild(createPostElement(post)));
                postsDiv.appendChild(ul);
            });
        })
        .catch(error => {
            console.error('Error fetching posts:', error);
            const postsDiv = document.getElementById('posts');
            postsDiv.innerHTML = '<center>Error loading posts. Please try again later.</center>';
        });
}

function createPostElement(post) {
    const li = document.createElement('li');
    li.className = 'post-container';

    const postLink = document.createElement('a');
    postLink._href = `/post/${post.id}`;
    postLink.className = 'post-item';
    const postContent = document.createElement('span');
    postContent.innerHTML = renderPostContent(post.sentence);
    postLink.appendChild(postContent);

    postLink.addEventListener('click', (event) => {
        if (event.target.tagName === 'A') {
            event.stopPropagation();
        } else {
            event.preventDefault();
            window.location.href = postLink._href;
        }
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