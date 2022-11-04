import React from "react";
import { createPortal } from "react-dom";
import { Close, ModalBody, ModalContent, ModalHead } from "./style";

/* 
  This modal component is attached directly to the <body> element of the html file, 
  this behavior is only triggered when modal is opened.
*/

interface CustomModalProps {
  show?: boolean;
  toggleModal?: any;
  borderRadius?: string;
  heading?: string;
  styles?: React.CSSProperties | undefined;
  children?: React.ReactNode;
}

const CustomModal = (props: CustomModalProps) => {
  const { show, toggleModal, borderRadius, heading, styles, children } = props;

  const handleClickOutside = (e: any) => {
    if (e.target === e.currentTarget) {
      toggleModal();
    }
  };

  if (show) {
    return createPortal(
      <ModalBody show={show} onMouseDown={handleClickOutside} style={styles}>
        <ModalContent borderRadius={borderRadius}>
          <ModalHead>
            <h2>{heading}</h2>
            <Close
              onClick={() => toggleModal(!show)}
              src={require("../../assets/icons/close-icon.png")}
            />
          </ModalHead>
          {children}
        </ModalContent>
      </ModalBody>,
      document.body
    );
  } else {
    return <></>;
  }
};
export default CustomModal;
