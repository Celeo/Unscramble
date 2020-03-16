module Main exposing (main)

import Browser
import Html exposing (Html, button, div, hr, input, p, text)
import Html.Attributes exposing (placeholder, value)
import Html.Events exposing (onClick, onInput)
import Http
import Json.Decode exposing (Decoder, field, list, string)


main : Program () Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = \_ -> Sub.none
        }


type alias Model =
    { word : String
    , matches : List String
    , errorMessage : Maybe String
    }


init : () -> ( Model, Cmd Msg )
init _ =
    ( { word = ""
      , matches = []
      , errorMessage = Nothing
      }
    , Cmd.none
    )


type Msg
    = SetWord String
    | SearchForMatches
    | DataReceived (Result Http.Error String)


url : String
url =
    "http://localhost:3030/word/"


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        SetWord value ->
            ( { model | word = value }, Cmd.none )

        SearchForMatches ->
            ( model, Http.get { url = url ++ model.word, expect = Http.expectString DataReceived } )

        DataReceived result ->
            case result of
                Ok data ->
                    -- ( { model | word = data }, Cmd.none )
                    -- TODO need to use the JSON decoder here
                    ( model, Cmd.none )

                Err _ ->
                    ( { model | errorMessage = Just "Could not get data from server" }, Cmd.none )


view : Model -> Html Msg
view model =
    div []
        [ p [] [ text ("Current word: " ++ model.word) ]
        , input [ placeholder "Letters to unscramble", value model.word, onInput SetWord ] []
        , button [ onClick SearchForMatches ] [ text "Search" ]
        , hr [] []
        , p [] [ text ("Matches: " ++ Debug.toString model.matches) ]
        ]
