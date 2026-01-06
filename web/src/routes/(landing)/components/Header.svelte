<script lang="ts">
	import Logomark from "$lib/assets/logomark.svg?raw"
	import Menu from "$lib/assets/menu.svg?raw"
	import Button from "$lib/components/Button.svelte"
	import * as DropdownMenu from "$lib/components/dropdown-menu"
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

	const dropdownMenuItems = [{ title: "Log in", href: "/login" }, ...navItems]
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
	<DropdownMenu.Root>
		<DropdownMenu.Trigger class="flex lg:hidden">
			{@html Menu}
		</DropdownMenu.Trigger>
		<DropdownMenu.Content class="flex flex-col lg:hidden">
			{#each dropdownMenuItems as { title, href }}
				<DropdownMenu.Item>
					{#snippet child({ props })}
						<a {...props} {href}>
							{title}
						</a>
					{/snippet}
				</DropdownMenu.Item>
			{/each}
		</DropdownMenu.Content>
	</DropdownMenu.Root>
	<Button intent="secondary" class="hidden duration-300 lg:flex" href="/login">Log in</Button>
</header>
