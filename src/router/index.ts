import { createRouter, createMemoryHistory } from "vue-router";
import WizardLayout from "@/components/WizardLayout.vue";
import ChooseEdition from "@/components/steps/ChooseEdition.vue";
import ConfigureEdition from "@/components/steps/ConfigureEdition.vue";
import CreateSDCard from "@/components/steps/CreateSDCard.vue";
import HomeLayout from "@/components/HomeLayout.vue";
import ChooseImage from "@/components/steps/ChooseImage.vue";
import ChooseDisk from "@/components/steps/ChooseDisk.vue";

const router = createRouter({
  history: createMemoryHistory(),
  routes: [
    {
      path: "/",
      components: {
        default: HomeLayout
      },
      children: [
        {
          path: "",
          name: "choose-image",
          components: {
            default: ChooseImage
          },
        },
        {
          path: "",
          name: "choose-disk",
          components: {
            default: ChooseDisk
          },
        },
        // {
        //   path: "configure/:edition",
        //   name: "configure-edition",
        //   components: {
        //     default: ConfigureEdition,
        //   },
        // },
        // {
        //   path: "sd-card",
        //   name: "sd-card",
        //   components: {
        //     default: CreateSDCard,
        //   },
        // },
      ],
    },
  ],
});

export default router;
