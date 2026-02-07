<script lang="ts">
	import Logomark from "$lib/assets/logomark.svg?raw"
	import Sidebar from "$lib/assets/sidebar.svg?raw"
	import Button from "$lib/components/Button.svelte"
	import Link from "$lib/components/Link.svelte"
	import * as Sheet from "$lib/components/sheet"

	const navLinks = [
		{ title: "Docs", href: "/" },
		{ title: "Repo", href: "https://github.com/aanesn/lambda" },
		{ title: "Contact", href: "mailto:contact@lambda.new" }
	]
</script>

<header class="flex h-16 items-center justify-between lg:h-20">
	<a href="/">
		{@html Logomark}
	</a>
	<nav class="absolute left-1/2 hidden -translate-x-1/2 items-center gap-x-8 lg:flex">
		{#each navLinks as { title, href }}
			<Link intent="secondary" {href}>{title}</Link>
		{/each}
	</nav>
	<Button intent="secondary" class="hidden duration-300 lg:flex" href="/login">Log in</Button>
	<Sheet.Root>
		<Sheet.Trigger>
			{#snippet child({ props })}
				<Button {...props} intent="ghost" size="icon" class="flex lg:hidden">
					{@html Sidebar}
				</Button>
			{/snippet}
		</Sheet.Trigger>
		<Sheet.Content class="flex w-72 p-6 lg:hidden">
			<div class="pb-4">
				{@html Logomark}
			</div>
			<div class="flex flex-col gap-y-1">
				{#each [...navLinks, { title: "Log in", href: "/login" }] as { title, href }}
					<Link {href} class="text-lg">{title}</Link>
				{/each}
			</div>
		</Sheet.Content>
	</Sheet.Root>
</header>
