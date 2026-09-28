from pathlib import Path
import markdown

BLOG_DIR = Path("blog")

def parse_post(path):
    text = path.read_text(encoding="utf-8")

    parts = text.split("---", 2)

    if (len(parts) != 3):
        raise ValueError("Too many section splits at {path}")

    front_matter = parts[1].strip()
    content = parts[2].strip()

    metadata = {}

    for line in front_matter.splitlines():
        key, value = line.split(":",1)
        metadata[key.strip()] = value.strip()

    return metadata, content

def generate_post(path):
    metadata, content = parse_post(path)

    html_content = markdown.markdown(
        content,
        extensions=["fenced_code"]
    )

    output = f"""<!DOCTYPE html>
<html>
<head>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" href="../../css/style.css">
    <title>{metadata["title"]} | anacondasniper.net</title>
</head>
<body>
    <div class="topnav">
        <h1 class="topnav-title">anacondasniper.net</h1>
        <div class="navlinks">
            <a href="../../index.html">Home</a>
            <a href="../../projects.html">Projects</a>
            <a class="active" href="../../blog.html">Blog</a>
        </div>
    </div>
    <article class="blogpost">
        <h1>{metadata["title"]}</h1>
        <p class="blogpost-date">
            {metadata["date"]} · {metadata["tag"]}
        </p>

        {html_content}

        <a class="back-btn" href="blog.html">
        ← Back
        </a>
    </article>
</body>
</html>
    """

    output_path = path.with_suffix(".html")
    output_path.write_text(output, encoding="utf-8")

    print(f"Generated {output_path}")

def main():
    posts = BLOG_DIR.rglob("*.md")

    for post in posts:
        generate_post(post)

if __name__ == "__main__":
    main()
