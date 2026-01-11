<script lang="ts">
	import Logomark from "$lib/assets/logomark.svg?raw"
	import Sidebar from "$lib/assets/sidebar.svg?raw"
	import Button from "$lib/components/Button.svelte"
	import Link from "$lib/components/Link.svelte"
	import * as Sheet from "$lib/components/sheet"
	import { SIDEBAR_WIDTH } from "$lib/utils"

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

	const sheetItems = [
		{ title: "Start now", href: "/register" },
		{ title: "Log in", href: "/login" },
		...navItems
	]
</script>

<header class="relative flex h-16 items-center justify-between lg:h-20">
	<a href="/">
		{@html Logomark}
	</a>
	<nav class="absolute left-1/2 hidden -translate-x-1/2 items-center gap-x-8 lg:flex">
		{#each navItems as { title, href }}
			<Link {href}>{title}</Link>
		{/each}
	</nav>
	<Button intent="secondary" class="hidden duration-300 lg:flex" href="/login">Log in</Button>
	<Sheet.Root>
		<Sheet.Trigger class="flex lg:hidden">
			{#snippet child({ props })}
				<Button {...props} intent="ghost" size="icon">
					{@html Sidebar}
				</Button>
			{/snippet}
		</Sheet.Trigger>
		<Sheet.Content
			class="flex w-(--sidebar-width) p-2 lg:hidden"
			style="--sidebar-width: {SIDEBAR_WIDTH};"
		>
			<Sheet.Header>
				{@html Logomark}
			</Sheet.Header>
			<div class="flex h-full w-full flex-col items-start gap-2 px-4">
				{#each sheetItems as { title, href }}
					<Link {href} class="text-base">{title}</Link>
				{/each}
			</div>
		</Sheet.Content>
	</Sheet.Root>
</header>
