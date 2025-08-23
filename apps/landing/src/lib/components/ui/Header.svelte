<script lang="ts">
	import Logomark from "../icons/Logomark.svelte"
	import Button from "./Button.svelte"
	import Link from "./Link.svelte"
</script>

<header
	class="sticky top-0 z-50 flex h-16 items-center justify-between bg-black/80 backdrop-blur-lg"
>
	<Logomark />
	<div class="flex items-center gap-x-6">
		<Button class="duration-300">Start now</Button>
		<Link class="duration-300" href="/">Docs</Link>
	</div>
	<hr
		class="absolute bottom-0 h-px w-full border-0 bg-gradient-to-r from-transparent via-white/10 to-transparent"
	/>
</header>
