<template>
  <div class="bg-blue-800 md:overflow-hidden">
    <div class="px-4 py-20 md:py-4">
      <div class="md:max-w-6xl md:mx-auto">
        <div class="md:flex md:flex-wrap">
          <div class="md:w-1/2 text-center md:text-left md:pt-16">
            <h1 class="font-bold text-white text-2xl md:text-5xl leading-tight mb-4">
              Squashing bugs has never been easier
            </h1>

            <p class="text-indigo-200 md:text-xl md:pr-48">
              Lorem ipsum, dolor sit amet consectetur adipisicing elit. Id
              vitae corrupti asperiores veritatis dolorum, commodi aperiam
              enim.
            </p>

            <a href="#"
              class="mt-6 mb-12 md:mb-0 md:mt-10 inline-block py-3 px-8 text-white bg-red-500 hover:bg-red-600 rounded-lg shadow">
              Get Started
            </a >
          </div>
          <div class="md:w-1/2 relative">
            <div class="hidden md:block">
              <div class="-ml-24 -mb-40 absolute left-0 bottom-0 w-40 bg-white rounded-lg shadow-lg px-6 py-8"
                style="transform: rotate(-8deg);">
                <div class="bg-blue-800 mx-auto rounded-lg px-2 pb-2 relative mb-8" >
                  <div class="mb-1">
                    <span class="w-1 h-1 bg-indigo-100 rounded-full inline-block" style="margin-right: 1px;" ></span >
                    <span class="w-1 h-1 bg-indigo-100 rounded-full inline-block" style="margin-right: 1px;" ></span >
                    <span class="w-1 h-1 bg-indigo-100 rounded-full inline-block" ></span>
                  </div>
                  <div class="h-1 w-12 bg-indigo-100 rounded mb-1"></div>
                  <div class="h-1 w-10 bg-indigo-100 rounded mb-2"></div>

                  <div class="flex">
                    <div class="w-6 h-3 rounded bg-indigo-100 mr-1"></div>
                    <div class="w-6 h-3 rounded bg-indigo-100"></div>
                  </div>

                  <div class="-mr-2 -mb-4 absolute bottom-0 right-0 h-16 w-10 rounded-lg bg-green-700 border-2 border-white" > </div>
                  <div class="w-2 h-2 rounded-full bg-green-800 mx-auto absolute bottom-0 right-0 mr-2 -mb-2 z-10 border-2 border-white" > </div>
                </div>

                <div class="text-gray-800 text-center">
                  Unit<br/>Testing
                </div>
              </div>

              <div class="w-full absolute left-0 bottom-0 ml-1" style="transform: rotate(-8deg); z-index: 1; margin-bottom: -360px;">
                <div class="grid--gray h-48 w-48"></div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <svg class="fill-current text-white hidden md:block" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1440 320" >
      <path fill-opacity="1" d="M0,224L1440,32L1440,320L0,320Z"></path>
    </svg>
  </div>
</template>

<script>
export default {
  name: 'Landing'
}
</script>
