<script>
  const footerLinks = [
    { href: "/privacy", text: "Privacy" },
    { href: "/terms", text: "Terms" },
    { href: "https://github.com/pindjouf/cryptoooor", text: "Source Code" }
  ];
</script>

<footer>
  <div class="footer-content">
    <div class="footer-links">
      {#each footerLinks as { href, text }}
        <a {href}>{text}</a>
      {/each}
    </div>
    <p class="copyright">© 2024 Cryptoooor. No rights reserved.</p>
  </div>
</footer>
