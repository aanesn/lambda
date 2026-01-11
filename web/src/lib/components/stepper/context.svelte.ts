import { getContext, setContext } from "svelte"

type Getter<T> = () => T

export type StepperStateProps = {
	curr: Getter<number>
	setCurr: (curr: number) => void
	heights: Getter<number[]>
}

class StepperState {
	readonly props: StepperStateProps
	curr = $derived.by(() => this.props.curr())
	setCurr: StepperStateProps["setCurr"]
	height = $derived.by(() => this.props.heights()[this.curr] ?? 0)
	constructor(props: StepperStateProps) {
		this.props = props
		this.setCurr = props.setCurr
	}
	isActive = (i: number) => i === this.curr
	isPrev = (i: number) => i === this.curr - 1
	goto = (i: number) => this.setCurr(i)
}

const SYMBOL_KEY = "stepper"

export function setStepper(props: StepperStateProps): StepperState {
	return setContext(Symbol.for(SYMBOL_KEY), new StepperState(props))
}

export function useStepper(): StepperState {
	return getContext(Symbol.for(SYMBOL_KEY))
}
