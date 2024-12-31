<script>
  import { fade } from 'svelte/transition';
</script>

<svelte:head>
  <title>Privacy Policy - Cryptoooor's Zero-Data Commitment</title>
  <meta name="description" content="Cryptoooor's privacy policy - No tracking, no cookies, no data collection. Learn how we keep your files completely private with client-side encryption." />
  <meta property="og:title" content="Cryptoooor Privacy Policy" />
  <meta property="og:description" content="Zero data collection, zero tracking, zero compromise. Your privacy is our priority." />
  <meta property="og:type" content="website" />
  <meta property="og:image" content="/privacy-og-image.jpg" />
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="keywords" content="privacy policy, data protection, zero tracking, client-side encryption, file privacy" />
  <link rel="canonical" href="https://cryptoooor.com/privacy" />
</svelte:head>

<div class="privacy-page" in:fade>
  <h1>Privacy Policy</h1>
  
  <section class="policy-section">
    <h2>Our Commitment</h2>
    <p>Cryptoooor is designed with privacy at its core. We collect absolutely no data about you or your files.</p>
  </section>

  <section class="policy-section">
    <h2>How It Works</h2>
    <ul>
      <li>All encryption happens locally in your browser</li>
      <li>Files never leave your device</li>
      <li>No cookies or tracking</li>
      <li>No analytics or user data collection</li>
      <li>No server-side processing</li>
    </ul>
  </section>

  <section class="policy-section">
    <h2>Technical Details</h2>
    <p>The encryption process uses WebAssembly to run entirely in your browser. The encryption key never leaves your device, and the encrypted files are downloaded directly to your computer.</p>
  </section>

  <section class="policy-section">
    <h2>Contact</h2>
    <p>For privacy concerns or questions, reach out on <a href="https://x.com/pindjouf" target="_blank">X/Twitter</a>.</p>
  </section>
</div>

<style>
  .privacy-page {
    max-width: 800px;
    padding: 2rem;
    backdrop-filter: blur(12px);
  }

  h1 {
    text-align: center;
    margin-bottom: 2rem;
  }

  .policy-section {
    margin-bottom: 3rem;
  }

  .policy-section:last-child {
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
