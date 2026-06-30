<script lang="ts">
	import Logomark from "$lib/assets/logomark.svg?raw"
	import Menu from "$lib/assets/menu.svg?raw"
	import Button from "$lib/components/Button.svelte"
	import Link from "$lib/components/Link.svelte"
	import * as DropdownMenu from "$lib/components/dropdown-menu"

	const navLinks = [
		{ title: "repo", href: "https://github.com/aanesn/lambda" },
		{ title: "contact", href: "mailto:contact@lambda.new" }
	]

	const dropdownMenuLinks = [{ title: "start now", href: "/" }, ...navLinks]
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
	<Button class="duration-300 hidden lg:flex">start now</Button>
	<DropdownMenu.Root>
		<DropdownMenu.Trigger class="lg:hidden">{@html Menu}</DropdownMenu.Trigger>
		<DropdownMenu.Content class="lg:hidden">
			{#each dropdownMenuLinks as { title, href }}
				<DropdownMenu.Item>
					{#snippet child({ props })}
						<a {href} {...props}>{title}</a>
					{/snippet}
				</DropdownMenu.Item>
			{/each}
		</DropdownMenu.Content>
	</DropdownMenu.Root>
</header>
