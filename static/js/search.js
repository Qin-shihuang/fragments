function setupSearch() {
    const searchForm = document.getElementById('searchForm');
    const searchInput = document.getElementById('searchInput');

    searchForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        const searchTerm = searchInput.value.trim();
        if (searchTerm) {
            window.location.href = `/search?query=${encodeURIComponent(searchTerm)}`;
        } else {
            displaySearchResultPage(searchTerm, '');
        }
    });
}

function prepareSearchResultPage() {
    const urlParams = new URLSearchParams(window.location.search);
    const searchTerm = urlParams.get('query');
    if (searchTerm) {
        fetch(`/api/search?query=${encodeURIComponent(searchTerm)}`)
            .then(response => response.json())
            .then(posts => displaySearchResultPage(searchTerm, posts))
            .catch(error => {
                console.error('Error searching posts:', error);
                const postsDiv = document.getElementById('posts');
                postsDiv.innerHTML = '<center>Error searching posts. Please try again later.</center>';
            });
    }
}

function displaySearchResultPage(searchTerm, posts) {
    try {
        window.history.pushState({ posts }, '', '/search?query=' + encodeURIComponent(searchTerm));
        if (!searchTerm) {
            const postsDiv = document.getElementById('posts');
            postsDiv.innerHTML = '<center>No search term provided.</center>';
            return;
        }
        if (!posts || posts.length === 0) {
            const postsDiv = document.getElementById('posts');
            postsDiv.innerHTML = '<center>No posts match the selected criteria.</center>';
            return;
        }
        const postsDiv = document.getElementById('posts');
        postsDiv.innerHTML = '';
        const dateHeader = document.createElement('h2');
        dateHeader.textContent = `Search results for: ${searchTerm}`;
        postsDiv.appendChild(dateHeader);
        const hr = document.createElement('hr');
        hr.className = 'h2-hr';
        postsDiv.appendChild(hr);
        const ul = document.createElement('ul');
        posts.forEach(post => ul.appendChild(createPostElement(post)));
        postsDiv.appendChild(ul);
    } catch (error) {
        console.error('Error preparing search results:', error);
        postsDiv.innerHTML = '<center>Error preparing search results. Please try again later.</center>';
    }
}