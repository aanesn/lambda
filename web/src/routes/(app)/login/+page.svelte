<script lang="ts">
	import Github from "$lib/assets/github.svg?raw"
	import Google from "$lib/assets/google.svg?raw"
	import Logomark from "$lib/assets/logomark.svg?raw"
	import Button from "$lib/components/Button.svelte"
	import * as Card from "$lib/components/card"
	import Link from "$lib/components/Link.svelte"
	import { apiUrl } from "$lib/utils"
	import { goto } from "$app/navigation"
	import type { PageProps } from "./$types"

	let { data }: PageProps = $props()

	$effect(() => {
		data.user && goto("/dashboard", { replaceState: true })
	})
</script>

<svelte:head>
	<title>Login | Lambda.new</title>
	<meta name="description" content="Login or register" />
</svelte:head>

<div class="mx-auto max-w-lg px-6 pt-16">
	<div class="mb-5 ml-10">
		{@html Logomark}
	</div>
	<Card.Root>
		<Card.Header class="text-center">
			<Card.Title class="text-lg">Login or register</Card.Title>
			<Card.Description>Select a provider to continue to the dashboard</Card.Description>
		</Card.Header>
		<Card.Content class="flex flex-col items-center gap-y-3">
			<Button
				intent="outline"
				size="lg"
				class="w-full max-w-90"
				href={`${apiUrl}/auth/google`}
			>
				{@html Google}
				Continue with Google
			</Button>
			<Button
				intent="outline"
				size="lg"
				class="w-full max-w-90"
				href={`${apiUrl}/auth/github`}
			>
				{@html Github}
				Continue with Github
			</Button>
		</Card.Content>
		<Card.Footer class="justify-center">
			<Link intent="underline" href="/legal">Privacy & terms</Link>
		</Card.Footer>
	</Card.Root>
</div>
