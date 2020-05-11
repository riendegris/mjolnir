import { shallowMount } from '@vue/test-utils'
import Home from '@/components/Home/Home'
import Landing from '@/components/Home/Landing'

describe('Home.vue', () => {
  it('renders correct subcomponents', () => {
    const wrapper = shallowMount(Home, {})
    const contents = [Landing]

    contents.forEach(comp => {
      expect(wrapper.find(comp).exists())
    })
  })
})
