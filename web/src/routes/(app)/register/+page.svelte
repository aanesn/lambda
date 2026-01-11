<script lang="ts">
	import Github from "$lib/assets/github.svg?raw"
	import Google from "$lib/assets/google.svg?raw"
	import Button from "$lib/components/Button.svelte"
	import * as Card from "$lib/components/card"
	import Link from "$lib/components/Link.svelte"
	import * as Stepper from "$lib/components/stepper"
	import { apiUrl } from "$lib/utils"
	import { goto } from "$app/navigation"
	import type { PageProps } from "./$types"

	let { data }: PageProps = $props()

	$effect(() => {
		data.user && goto("/dashboard", { replaceState: true })
	})

	let curr = $state(0)
</script>

<svelte:head>
	<title>Register | Lambda.new</title>
	<meta name="description" content="Create your account" />
</svelte:head>

<Stepper.Root bind:curr heights={[280]}>
	<Stepper.Bg />
	<Stepper.Title>Register</Stepper.Title>
	<Stepper.Content>
		<Stepper.Item i={0}>
			<Card.Root>
				<Card.Header class="text-center">
					<Card.Title class="text-lg">Create your account</Card.Title>
					<Card.Description>Select a provider to create your account</Card.Description>
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
					<span class="text-sm text-neutral-400">
						Already have an account?
						<Link intent="underline" href="/login">Log in</Link>
					</span>
				</Card.Footer>
			</Card.Root>
		</Stepper.Item>
	</Stepper.Content>
</Stepper.Root>
