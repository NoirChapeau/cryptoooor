<script>
    const links = [
      { href: "/", text: "Home" },
      { href: "/about", text: "About" },
      { href: "https://x.com/pindjouf", text: "Contact", external: true }
    ];
</script>

<nav>
  {#each links as { href, text, external }}
    <a 
      {href} 
      class="nav-link" 
      target={external ? "_blank" : undefined} 
      rel={external ? "noopener noreferrer" : undefined}
    >
      {text}
    </a>
  {/each}
</nav>
