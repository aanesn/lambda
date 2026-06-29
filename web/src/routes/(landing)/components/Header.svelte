<script lang="ts">
	import Logomark from "$lib/assets/logomark.svg?raw"
	import Button from "$lib/components/Button.svelte"
	import Link from "$lib/components/Link.svelte"

	const navLinks = [
		{ title: "repo", href: "https://github.com/aanesn/lambda" },
		{ title: "contact", href: "mailto:contact@lambda.new" }
	]
</script>

<header class="flex items-center justify-between h-16 relative">
	<a href="/">
		{@html Logomark}
	</a>
	<nav class="hidden lg:flex items-center gap-x-8 absolute left-1/2 -translate-x-1/2">
		{#each navLinks as { title, href }}
			<Link {href}>{title}</Link>
		{/each}
	</nav>
	<Button class="duration-300">start now</Button>
</header>
