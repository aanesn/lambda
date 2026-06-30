<script lang="ts">
	import Footer from "./components/Footer.svelte"
	import Header from "./components/Header.svelte"

	let { children } = $props()
</script>

<div class="container mx-auto px-6 xl:max-w-7xl min-h-screen flex flex-col">
	<Header />
	<div class="flex-1">
		{@render children()}
	</div>
	<Footer />
</div>
