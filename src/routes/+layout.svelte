<script>
import '../app.css';
import Nav from '$lib/components/Nav.svelte';
import Footer from '$lib/components/Footer.svelte';
</script>

<div class="layout-wrapper" role="document">
    <header>
        <Nav />
    </header>
    <main>
        <slot />
    </main>
    <Footer />
</div>
