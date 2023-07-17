import { extendTheme } from "@chakra-ui/react"
import Button from './button';
import Modal from './modal';
import Table from './table';
import Divider from './divider';
import Alert from './alert';

const theme = extendTheme({
  style: {
    global: {
      background: "black",
      body: {
        fontColor: "white",
      }
    }
  },
  fonts: {
    heading: `'Gotham', 'Montserrat', sans-serif`,
    body:`'Gotham', 'Montserrat', sans-serif`,
  },
  components: {
    Button,
    Modal,
    Table,
    Divider,
    Alert,
  },
  colors: {
    brandGreen: "#3CCD64",
    lightGray: "#A9A9A9",
    darkGray: "#1A1A1A",
    darkTransparentGray: "rgba(0, 0, 0, 0.5)",
    transparentGray: "rgba(0, 0, 0, 0.25)",
    lightTransparentGray: "rgba(255, 255, 255, 0.1)",
  }
})

export default theme;