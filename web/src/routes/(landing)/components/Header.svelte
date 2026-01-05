<script lang="ts">
	import Logomark from "$lib/assets/logomark.svg?raw"
	import Button from "$lib/components/Button.svelte"
	import Link from "$lib/components/Link.svelte"

	const navItems = [
		{
			title: "Documentation",
			href: "/docs"
		},
		{
			title: "Pricing",
			href: "/pricing"
		},
		{
			title: "Feedback",
			href: "mailto:contact@lambda.new"
		}
	]
</script>

<header class="relative flex h-16 items-center justify-between lg:h-18">
	<a href="/">
		{@html Logomark}
	</a>
	<nav class="absolute left-1/2 hidden -translate-x-1/2 items-center gap-x-8 lg:flex">
		{#each navItems as { title, href }}
			<Link {href}>{title}</Link>
		{/each}
	</nav>
	<Button intent="secondary" class="duration-300" href="/login">Log in</Button>
</header>
