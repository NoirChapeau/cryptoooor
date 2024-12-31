<script>
  import { fade } from 'svelte/transition';
</script>

<svelte:head>
  <title>Terms of Service - Cryptoooor's User Agreement</title>
  <meta name="description" content="Cryptoooor's terms of service. Free, open-source encryption tool provided as-is. Learn about usage terms, warranties, and limitations." />
  <meta property="og:title" content="Cryptoooor Terms of Service" />
  <meta property="og:description" content="Understand Cryptoooor's terms of use, warranties, and acceptable usage policy." />
  <meta property="og:type" content="website" />
  <meta property="og:image" content="/terms-og-image.jpg" />
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="keywords" content="terms of service, user agreement, encryption tool terms, usage policy, open source license" />
  <link rel="canonical" href="https://cryptoooor.com/terms" />
</svelte:head>

<div class="terms-page" in:fade>
  <header>
      <h1>Terms of Service</h1>
  <header>

  <section class="terms-section">
    <h2>Overview</h2>
    <p>Cryptoooor is provided as-is, free of charge. By using Cryptoooor, you agree to these terms.</p>
  </section>

  <section class="terms-section">
    <h2>No Warranties</h2>
    <p>While we strive for excellence, Cryptoooor comes with no warranties. You use it at your own risk. Always keep backups of your files.</p>
  </section>

  <section class="terms-section">
    <h2>Acceptable Use</h2>
    <p>You agree to:</p>
    <ul>
      <li>Use Cryptoooor legally and responsibly</li>
      <li>Not attempt to reverse engineer or modify the service</li>
      <li>Not use Cryptoooor for malicious purposes</li>
    </ul>
  </section>

  <section class="terms-section">
    <h2>Limitation of Liability</h2>
    <p>We are not liable for any data loss or damages resulting from using Cryptoooor. Always maintain backups of your important files.</p>
  </section>

  <section class="terms-section">
    <h2>Open Source</h2>
    <p>Cryptoooor is open-source software. You can find the source code on <a href="https://github.com/pindjouf/cryptoooor" target="_blank">GitHub</a>.</p>
  </section>
</div>

<style>
  .terms-page {
    max-width: 800px;
    padding: 2rem;
    backdrop-filter: blur(12px);
  }

  h1 {
    text-align: center;
    margin-bottom: 2rem;
  }

  .terms-section {
    margin-bottom: 3rem;
  }

  .terms-section:last-child {
    margin-bottom: 0;
  }

  h2 {
    color: var(--accent);
    margin-bottom: 1rem;
  }

  ul {
    list-style-type: none;
    padding: 0;
  }

  li {
    margin-bottom: 0.5rem;
    padding-left: 1.5rem;
    position: relative;
  }

  li::before {
    content: "•";
    color: var(--accent);
    position: absolute;
    left: 0;
  }

  a {
    color: var(--accent);
    text-decoration: none;
  }

  a:hover {
    text-decoration: underline;
  }
</style>
