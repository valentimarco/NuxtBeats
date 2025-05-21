export default defineAppConfig({
  // https://ui.nuxt.com/getting-started/theme#design-system
  ui: {
    colors: {
      primary: "purple",
      secondary: "green",
      neutral: "zinc",
      success: "green",
      info: "sky",
      warning: "amber",
      error: "pink",
    },
    button: {
      slots: {
        base: "cursor-pointer disabled:cursor-not-allowed transition-all",
      },
    },
    checkbox: {
      slots: {
        base: "transition-colors",
      },
    },
    stepper: {
      slots: {
        trigger: "cursor-pointer disabled:cursor-not-allowed disabled:bg-(--ui-bg-muted) transition-colors",
        separator: "transition-colors",
      },
    },
    input: {
      slots: {
        base: "transition-all",
      },
      variants: {
        size: {
          xs: {
            trailing: "pe-0.5",
          },
          sm: {
            trailing: "pe-1",
          },
          md: {
            trailing: "pe-1.5",
          },
          lg: {
            trailing: "pe-2",
          },
          xl: {
            trailing: "pe-2.5",
          },
        },
      },
    },
    inputMenu: {
      slots: {
        base: "transition-all",
        tagsItemDelete: "cursor-pointer hover:bg-transparent hover:text-(--ui-error)",
      },
    },
    formField: {
      slots: {
        description: "text-balance",
      },
    },
    slideover: {
      variants: {
        side: {
          right: {
            content: "max-w-lg",
          },
          left: {
            content: "max-w-lg",
          },
        },
      },
    },
    select: {
      slots: {
        content: "w-fit min-w-(--reka-popper-anchor-width) max-w-64",
        trailingIcon: "group-data-[state=open]:rotate-180 transition-transform duration-200",
      },
    },
    selectMenu: {
      slots: {
        content: "w-fit min-w-(--reka-popper-anchor-width) max-w-64",
        trailingIcon: "group-data-[state=open]:rotate-180 transition-transform duration-200",
      },
      compoundVariants: [
        {
          leading: true,
          size: "md",
          class: "ps-10",
        },
        {
          trailing: true,
          size: "md",
          class: "pe-10",
        },
      ],
    },
    tabs: {
      slots: {
        trigger: ["cursor-pointer"],
      },
    },
    table: {
      slots: {
        base: "cursor-grab",
      },
      variants: {
        loading: {
          true: {
            thead: "after:h-0.5",
          },
        },
      },
    },
    navigationMenu: {
      slots: {
        link: "gap-3",
      },
      variants: {
        active: {
          false: {
            link: "text-(--ui-text-toned)",
            linkLeadingIcon: "text-(--ui-text-muted)",
          },
        },
      },
    },
    calendar: {
      slots: {
        cellTrigger: "rounded-lg",
      },
    },
    alert: {
      slots: {
        root: "overflow-clip",
        actions: "gap-2",
        title: "font-semibold",
      },
    },
    card: {
      slots: {
        header: "p-2 sm:px-2 md:p-4",
        body: "p-2 sm:p-2 md:p-4",
        footer: "p-2 sm:px-2 md:p-4",
      },
    },
    radioGroup: {
      slots: {
        base: "cursor-pointer",
      },
      variants: {
        size: {
          xs: {
            fieldset: "gap-1",
          },
          sm: {
            fieldset: "gap-1",
          },
          md: {
            fieldset: "gap-2",
          },
          lg: {
            fieldset: "gap-2",
          },
          xl: {
            fieldset: "gap-3",
          },
        },
      },
    },
    modal: {
      slots: {
        overlay: "backdrop-blur-sm",
      },
      variants: {
        fullscreen: {
          false: {
            content: "max-w-xl",
          },
        },
      },
    },
  },
})
