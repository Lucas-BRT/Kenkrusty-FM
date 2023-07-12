import yellowCross from "../../assets/YellowCross.svg"
import "./styles.css"


const ErrorMensage = (props) => {
    return (
        <div className="ErrorMensage">
            Error Conecting to Kenku FM
            <div className="Error">{props.error}</div>
        </div>
    )
}

export const ErrorPage = (props) => {
    return (
        
        <div className="ErrorFrame">
            <img src={yellowCross} alt="" />
            <ErrorMensage error={props.error} />
        </div>

    )
}






