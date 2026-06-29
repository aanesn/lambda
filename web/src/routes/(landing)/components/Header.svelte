<script lang="ts">
	import Logomark from "$lib/assets/logomark.svg?raw"
	import Button from "$lib/components/Button.svelte"
	import Link from "$lib/components/Link.svelte"

	const navLinks = [
		{ title: "repo", href: "https://github.com/aanesn/lambda" },
		{ title: "contact", href: "mailto:contact@lambda.new" }
	]
</script>

<header class="relative flex h-16 items-center justify-between">
	<a href="/">
		{@html Logomark}
	</a>
	<nav class="absolute left-1/2 hidden -translate-x-1/2 items-center gap-x-8 lg:flex">
		{#each navLinks as { title, href }}
			<Link {href}>{title}</Link>
		{/each}
	</nav>
	<Button class="duration-300">start now</Button>
</header>
