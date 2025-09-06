<script lang="ts">
	import Logomark from "../icons/Logomark.svelte"
	import Button from "./Button.svelte"
	import Link from "./Link.svelte"
</script>

<header class="flex h-[72px] items-center justify-between">
	<Logomark />
	<div class="flex items-center gap-x-6">
		<Button class="duration-300">Start now</Button>
		<Link class="duration-300" href="/">Docs</Link>
	</div>
</header>
