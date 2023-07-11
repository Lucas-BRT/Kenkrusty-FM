import loadingAnimation from "../../assets/YellowLoading.svg"
import "./styles.css"

export const LoadingPage = () => {
    return (
        <div className="loadingFrame">
            <div className="loadingAnimation">
                <img src={loadingAnimation} alt="" />
            </div>
            <div className="loadingText">
                Carregando...
            </div> 
        </div>
    )
}










