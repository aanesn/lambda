<script lang="ts">
	import Github from "$lib/assets/github.svg?raw"
	import Google from "$lib/assets/google.svg?raw"
	import Logomark from "$lib/assets/logomark.svg?raw"
	import Button from "$lib/components/Button.svelte"
	import * as Card from "$lib/components/card"
	import Link from "$lib/components/Link.svelte"
	import { apiUrl } from "$lib/utils"
	import { goto } from "$app/navigation"
	import type { PageData } from "./$types"

	let { data }: { data: PageData } = $props()

	$effect(() => {
		data.user && goto("/dashboard/new-project", { replaceState: true })
	})
</script>

<svelte:head>
	<title>Login | Lambda</title>
	<meta name="description" content="Log in or register" />
</svelte:head>

<div class="mx-auto mt-20 w-full max-w-xl px-6">
	<div class="mb-6 ml-12">
		{@html Logomark}
	</div>
	<Card.Root class="rounded-4xl">
		<Card.Header class="text-center">
			<Card.Title class="text-lg">Log in or register</Card.Title>
			<Card.Description>Select a provider to continue to the dashboard.</Card.Description>
		</Card.Header>
		<Card.Content class="flex flex-col items-center gap-y-3">
			<Button
				intent="outline"
				size="lg"
				class="w-full max-w-100"
				href={`${apiUrl}/auth/google`}
			>
				{@html Google}
				Continue with Google
			</Button>
			<Button
				intent="outline"
				size="lg"
				class="w-full max-w-100"
				href={`${apiUrl}/auth/github`}
			>
				{@html Github}
				Continue with Github
			</Button>
		</Card.Content>
		<Card.Footer class="flex justify-center">
			<Link intent="secondary" href="/" class="text-sm">Privacy & terms</Link>
		</Card.Footer>
	</Card.Root>
</div>
