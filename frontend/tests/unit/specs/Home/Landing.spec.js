import Landing from '@/components/Home/Landing'
import { shallowMount } from '@vue/test-utils'

describe('Landing.vue', () => {
  it('renders props.msg when passed', () => {
    const msg = 'Squashing bugs'
    const wrapper = shallowMount(Landing, {})
    expect(wrapper.text()).toMatch(msg)
  })
})
