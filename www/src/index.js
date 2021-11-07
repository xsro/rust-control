import * as demo1 from "./demo1/index";

const url = new URL(document.location);
const demoNumber = url.searchParams.get("demo");
const homepage=document.getElementById("homepage");

switch (demoNumber) {
    case "1":
        demo1.activate();
        break;
    case "2":
    default:
        homepage.hidden=false;
}
