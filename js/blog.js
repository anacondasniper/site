const postsContainer = document.getElementById("blogposts");
const filterButtons = document.querySelectorAll(".blogfilters button");

let posts = [];

async function loadPosts() {
	const response = await fetch("posts.json");
	posts = await response.json();

	displayPosts("All");
}

function displayPosts(tag) {
	postsContainer.innerHTML = "";

	const filteredPosts = tag === "All"
		? posts
		: posts.filter(post => post.tag === tag);

	filteredPosts.forEach(post => {
        const article = document.createElement("article");
        article.className = "blogpost";

        article.innerHTML = `
            <h3>${post.title}</h3>

            <p class="blogpost-date">
                ${post.date} · ${post.tag}
            </p>

            <a href="${post.url}">
                Read more →
            </a>
        `;

        postsContainer.appendChild(article);
    });
}

filterButtons.forEach(button => {
    button.addEventListener("click", () => {
        filterButtons.forEach(button => {
            button.classList.remove("active");
        });

        button.classList.add("active");

        displayPosts(button.dataset.tag);
    });
});


loadPosts();
