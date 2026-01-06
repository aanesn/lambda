<script lang="ts">
	import { DropdownMenu } from "bits-ui"
	import { cx } from "$lib/utils"

	let { class: className, ...restProps }: DropdownMenu.TriggerProps = $props()
</script>

<DropdownMenu.Trigger class={cx("focus:outline-none", className)} {...restProps} />
